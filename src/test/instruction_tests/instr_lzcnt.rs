use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 223], OperandSize::Word)
}

#[test]
fn lzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 47], OperandSize::Word)
}

#[test]
fn lzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 214], OperandSize::Dword)
}

#[test]
fn lzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(CX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 8], OperandSize::Dword)
}

#[test]
fn lzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 230], OperandSize::Qword)
}

#[test]
fn lzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RDI, 496659641, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 167, 185, 108, 154, 29], OperandSize::Qword)
}

#[test]
fn lzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 206], OperandSize::Word)
}

#[test]
fn lzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(DI, 86, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 117, 86], OperandSize::Word)
}

#[test]
fn lzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 210], OperandSize::Dword)
}

#[test]
fn lzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ECX, 1468612835, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 169, 227, 64, 137, 87], OperandSize::Dword)
}

#[test]
fn lzcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 225], OperandSize::Qword)
}

#[test]
fn lzcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1820290368, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 52, 117, 64, 109, 127, 108], OperandSize::Qword)
}

#[test]
fn lzcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 239], OperandSize::Qword)
}

#[test]
fn lzcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 19], OperandSize::Qword)
}

