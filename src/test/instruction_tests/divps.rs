use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn divps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 241], OperandSize::Dword)
}

fn divps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1840765098, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 162, 170, 216, 183, 109], OperandSize::Dword)
}

fn divps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 213], OperandSize::Qword)
}

fn divps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 50], OperandSize::Qword)
}

