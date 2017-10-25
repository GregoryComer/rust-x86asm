use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 156, 205, 244], OperandSize::Dword)
}

fn vrsqrt28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 205, 6], OperandSize::Dword)
}

fn vrsqrt28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 189, 153, 205, 196], OperandSize::Qword)
}

fn vrsqrt28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 129, 205, 25], OperandSize::Qword)
}

