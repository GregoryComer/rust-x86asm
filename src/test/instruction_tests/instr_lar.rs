use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 231], OperandSize::Word)
}

#[test]
fn lar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BX, 127, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 111, 127], OperandSize::Word)
}

#[test]
fn lar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 225], OperandSize::Dword)
}

#[test]
fn lar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 20, 183], OperandSize::Dword)
}

#[test]
fn lar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 218], OperandSize::Qword)
}

#[test]
fn lar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BP)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 2, 42], OperandSize::Qword)
}

#[test]
fn lar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(DX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 210], OperandSize::Word)
}

#[test]
fn lar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 25], OperandSize::Word)
}

#[test]
fn lar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 228], OperandSize::Dword)
}

#[test]
fn lar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 668439511, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 2, 52, 117, 215, 147, 215, 39], OperandSize::Dword)
}

#[test]
fn lar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(RDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 2, 214], OperandSize::Qword)
}

#[test]
fn lar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LAR, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 408933783, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 2, 44, 85, 151, 213, 95, 24], OperandSize::Qword)
}

