use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 156, 212], OperandSize::Dword)
}

fn vfnmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1534049581, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 156, 140, 112, 45, 189, 111, 91], OperandSize::Dword)
}

fn vfnmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 156, 219], OperandSize::Qword)
}

fn vfnmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 115723219, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 156, 172, 122, 211, 203, 229, 6], OperandSize::Qword)
}

fn vfnmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 156, 200], OperandSize::Dword)
}

fn vfnmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 7], OperandSize::Dword)
}

fn vfnmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 156, 253], OperandSize::Qword)
}

fn vfnmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 156, 26], OperandSize::Qword)
}

fn vfnmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 156, 192], OperandSize::Dword)
}

fn vfnmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 935799012, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 156, 52, 213, 228, 40, 199, 55], OperandSize::Dword)
}

fn vfnmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 156, 52, 145], OperandSize::Dword)
}

fn vfnmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 101, 132, 156, 200], OperandSize::Qword)
}

fn vfnmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 117, 137, 156, 60, 80], OperandSize::Qword)
}

fn vfnmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RCX, 1229431294, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 148, 156, 177, 254, 161, 71, 73], OperandSize::Qword)
}

fn vfnmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 156, 203], OperandSize::Dword)
}

fn vfnmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 156, 35], OperandSize::Dword)
}

fn vfnmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 187, 156, 58], OperandSize::Dword)
}

fn vfnmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 85, 171, 156, 209], OperandSize::Qword)
}

fn vfnmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 45, 161, 156, 56], OperandSize::Qword)
}

fn vfnmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RAX, 849746719, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 21, 191, 156, 184, 31, 27, 166, 50], OperandSize::Qword)
}

fn vfnmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 154, 156, 241], OperandSize::Dword)
}

fn vfnmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1423172541, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 156, 44, 205, 189, 227, 211, 84], OperandSize::Dword)
}

fn vfnmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 22878408, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 217, 156, 188, 192, 200, 24, 93, 1], OperandSize::Dword)
}

fn vfnmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 117, 190, 156, 252], OperandSize::Qword)
}

fn vfnmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1851821067, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 13, 201, 156, 28, 253, 11, 140, 96, 110], OperandSize::Qword)
}

fn vfnmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 2062641318, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 222, 156, 28, 149, 166, 104, 241, 122], OperandSize::Qword)
}

