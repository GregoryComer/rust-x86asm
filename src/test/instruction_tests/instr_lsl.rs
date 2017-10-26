use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lsl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 255], OperandSize::Word)
}

#[test]
fn lsl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 69, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 104, 69], OperandSize::Word)
}

#[test]
fn lsl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 210], OperandSize::Dword)
}

#[test]
fn lsl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 444215035, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 172, 120, 251, 46, 122, 26], OperandSize::Dword)
}

#[test]
fn lsl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 229], OperandSize::Qword)
}

#[test]
fn lsl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RSI, 412227889, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 142, 49, 25, 146, 24], OperandSize::Qword)
}

#[test]
fn lsl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 218], OperandSize::Word)
}

#[test]
fn lsl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 133, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 163, 133, 0], OperandSize::Word)
}

#[test]
fn lsl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 220], OperandSize::Dword)
}

#[test]
fn lsl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1419303952, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 172, 190, 16, 220, 152, 84], OperandSize::Dword)
}

#[test]
fn lsl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 219], OperandSize::Qword)
}

#[test]
fn lsl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 262587349, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 172, 147, 213, 195, 166, 15], OperandSize::Qword)
}

#[test]
fn lsl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 236], OperandSize::Qword)
}

#[test]
fn lsl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 58], OperandSize::Qword)
}

