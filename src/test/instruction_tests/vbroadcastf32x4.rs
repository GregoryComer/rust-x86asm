use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 26, 20, 86], OperandSize::Dword)
}

fn vbroadcastf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 26, 4, 158], OperandSize::Qword)
}

fn vbroadcastf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1315013839, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 26, 28, 189, 207, 132, 97, 78], OperandSize::Dword)
}

fn vbroadcastf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 117820836, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 26, 172, 241, 164, 205, 5, 7], OperandSize::Qword)
}

