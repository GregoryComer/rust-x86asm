use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fmul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Memory(10321, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 14, 81, 40], OperandSize::Word)
}

fn fmul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 12, 83], OperandSize::Dword)
}

fn fmul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 2039223025, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 12, 253, 241, 18, 140, 121], OperandSize::Qword)
}

fn fmul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 203], OperandSize::Word)
}

fn fmul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Dword)
}

fn fmul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 202], OperandSize::Qword)
}

fn fmul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(DI, 21976, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 141, 216, 85], OperandSize::Word)
}

fn fmul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1143392337, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 140, 147, 81, 200, 38, 68], OperandSize::Dword)
}

fn fmul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 12, 72], OperandSize::Qword)
}

fn fmul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 202], OperandSize::Word)
}

fn fmul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 201], OperandSize::Dword)
}

fn fmul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 202], OperandSize::Qword)
}

