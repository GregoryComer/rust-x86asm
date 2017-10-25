use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshuff64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 203, 35, 196, 126], OperandSize::Dword)
}

fn vshuff64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1636269169, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 206, 35, 180, 146, 113, 124, 135, 97, 22], OperandSize::Dword)
}

fn vshuff64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 222, 35, 4, 151, 119], OperandSize::Dword)
}

fn vshuff64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 206, 35, 221, 30], OperandSize::Qword)
}

fn vshuff64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1883440119, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 253, 193, 35, 132, 195, 247, 3, 67, 112, 82], OperandSize::Qword)
}

fn vshuff64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1518688257, Some(OperandSize::Qword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 133, 218, 35, 132, 152, 1, 88, 133, 90, 55], OperandSize::Qword)
}

