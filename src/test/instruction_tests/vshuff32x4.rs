use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshuff32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 35, 252, 71], OperandSize::Dword)
}

fn vshuff32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 207, 35, 52, 243, 112], OperandSize::Dword)
}

fn vshuff32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 287856833, Some(OperandSize::Dword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 217, 35, 20, 149, 193, 88, 40, 17, 123], OperandSize::Dword)
}

fn vshuff32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 125, 194, 35, 243, 74], OperandSize::Qword)
}

fn vshuff32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 741635040, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 45, 202, 35, 188, 79, 224, 115, 52, 44, 110], OperandSize::Qword)
}

fn vshuff32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 93, 221, 35, 44, 158, 98], OperandSize::Qword)
}

