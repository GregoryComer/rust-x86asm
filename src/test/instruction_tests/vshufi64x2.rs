use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshufi64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 205, 67, 240, 37], OperandSize::Dword)
}

fn vshufi64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 171086295, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 203, 67, 180, 216, 215, 145, 50, 10, 112], OperandSize::Dword)
}

fn vshufi64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 308202247, Some(OperandSize::Qword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 221, 221, 67, 60, 149, 7, 203, 94, 18, 20], OperandSize::Dword)
}

fn vshufi64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM28)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 3, 189, 207, 67, 236, 44], OperandSize::Qword)
}

fn vshufi64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RAX, 1185540151, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 141, 194, 67, 128, 55, 232, 169, 70, 35], OperandSize::Qword)
}

fn vshufi64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1081029034, Some(OperandSize::Qword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 133, 222, 67, 44, 69, 170, 49, 111, 64, 18], OperandSize::Qword)
}

