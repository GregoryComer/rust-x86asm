use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 79, 215], OperandSize::Dword)
}

fn vrsqrt14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 860425341, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 79, 180, 112, 125, 12, 73, 51], OperandSize::Dword)
}

fn vrsqrt14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 69, 129, 79, 215], OperandSize::Qword)
}

fn vrsqrt14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 79, 44, 158], OperandSize::Qword)
}

