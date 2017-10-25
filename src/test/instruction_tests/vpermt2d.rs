use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermt2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 126, 201], OperandSize::Dword)
}

fn vpermt2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 126, 42], OperandSize::Dword)
}

fn vpermt2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 159, 126, 28, 89], OperandSize::Dword)
}

fn vpermt2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 29, 130, 126, 206], OperandSize::Qword)
}

fn vpermt2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 126, 30], OperandSize::Qword)
}

fn vpermt2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 61, 149, 126, 25], OperandSize::Qword)
}

fn vpermt2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 170, 126, 214], OperandSize::Dword)
}

fn vpermt2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1920944383, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 126, 156, 159, 255, 72, 127, 114], OperandSize::Dword)
}

fn vpermt2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 186, 126, 60, 194], OperandSize::Dword)
}

fn vpermt2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 37, 167, 126, 240], OperandSize::Qword)
}

fn vpermt2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RDX, 1256379731, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 21, 162, 126, 130, 83, 213, 226, 74], OperandSize::Qword)
}

fn vpermt2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 109, 191, 126, 4, 129], OperandSize::Qword)
}

fn vpermt2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 126, 244], OperandSize::Dword)
}

fn vpermt2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 91749535, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 205, 126, 44, 253, 159, 252, 119, 5], OperandSize::Dword)
}

fn vpermt2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 217, 126, 17], OperandSize::Dword)
}

fn vpermt2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 53, 205, 126, 219], OperandSize::Qword)
}

fn vpermt2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RAX, 1873659058, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 37, 203, 126, 136, 178, 196, 173, 111], OperandSize::Qword)
}

fn vpermt2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 126, 63], OperandSize::Qword)
}

