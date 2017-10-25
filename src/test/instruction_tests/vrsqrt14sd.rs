use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 79, 194], OperandSize::Dword)
}

fn vrsqrt14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 987923170, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 79, 188, 71, 226, 130, 226, 58], OperandSize::Dword)
}

fn vrsqrt14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 197, 137, 79, 221], OperandSize::Qword)
}

fn vrsqrt14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 245, 130, 79, 1], OperandSize::Qword)
}

