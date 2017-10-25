use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 238], OperandSize::Dword)
}

fn rsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 52, 64], OperandSize::Dword)
}

fn rsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 209], OperandSize::Qword)
}

fn rsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 1009722819, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 142, 195, 37, 47, 60], OperandSize::Qword)
}

