use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 239], OperandSize::Dword)
}

fn sqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 1836229341, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 170, 221, 162, 114, 109], OperandSize::Dword)
}

fn sqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 196], OperandSize::Qword)
}

fn sqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1348048072, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 52, 181, 200, 148, 89, 80], OperandSize::Qword)
}

