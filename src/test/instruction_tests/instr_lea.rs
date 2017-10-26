use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lea_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10223, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 176, 239, 39], OperandSize::Word)
}

#[test]
fn lea_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EBX, 275656669, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 147, 221, 47, 110, 16], OperandSize::Dword)
}

#[test]
fn lea_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 2076723471, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 12, 77, 15, 73, 200, 123], OperandSize::Qword)
}

#[test]
fn lea_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(DI, 39, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 85, 39], OperandSize::Word)
}

#[test]
fn lea_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 12, 81], OperandSize::Dword)
}

#[test]
fn lea_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 62], OperandSize::Qword)
}

#[test]
fn lea_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 2142184839, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 141, 148, 126, 135, 37, 175, 127], OperandSize::Qword)
}

