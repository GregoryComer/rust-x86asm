use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 156, 204, 245], OperandSize::Dword)
}

fn vrsqrt28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 204, 44, 150], OperandSize::Dword)
}

fn vrsqrt28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 204, 44, 151], OperandSize::Dword)
}

fn vrsqrt28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 125, 156, 204, 218], OperandSize::Qword)
}

fn vrsqrt28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(RCX, 628511209, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 204, 129, 233, 81, 118, 37], OperandSize::Qword)
}

fn vrsqrt28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 34489777, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 219, 204, 4, 245, 177, 69, 14, 2], OperandSize::Qword)
}

