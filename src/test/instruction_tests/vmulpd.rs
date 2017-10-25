use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 89, 224], OperandSize::Dword)
}

fn vmulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 89, 52, 240], OperandSize::Dword)
}

fn vmulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 89, 219], OperandSize::Qword)
}

fn vmulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 89, 28, 139], OperandSize::Qword)
}

fn vmulpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 89, 227], OperandSize::Dword)
}

fn vmulpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 60, 144], OperandSize::Dword)
}

fn vmulpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 246], OperandSize::Qword)
}

fn vmulpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 825697565, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 145, 29, 37, 55, 49], OperandSize::Qword)
}

fn vmulpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 139, 89, 255], OperandSize::Dword)
}

fn vmulpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 787560670, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 89, 28, 85, 222, 56, 241, 46], OperandSize::Dword)
}

fn vmulpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 496711688, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 158, 89, 4, 221, 8, 56, 155, 29], OperandSize::Dword)
}

fn vmulpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 205, 140, 89, 222], OperandSize::Qword)
}

fn vmulpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1898807058, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 149, 139, 89, 12, 221, 18, 127, 45, 113], OperandSize::Qword)
}

fn vmulpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RAX, 255938200, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 149, 159, 89, 144, 152, 78, 65, 15], OperandSize::Qword)
}

fn vmulpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 172, 89, 224], OperandSize::Dword)
}

fn vmulpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 171, 89, 54], OperandSize::Dword)
}

fn vmulpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 483615109, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 185, 89, 132, 67, 133, 97, 211, 28], OperandSize::Dword)
}

fn vmulpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 157, 165, 89, 242], OperandSize::Qword)
}

fn vmulpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RDX, 942931717, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 149, 161, 89, 178, 5, 255, 51, 56], OperandSize::Qword)
}

fn vmulpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1970901545, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 181, 189, 89, 164, 138, 41, 146, 121, 117], OperandSize::Qword)
}

fn vmulpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 251, 89, 242], OperandSize::Dword)
}

fn vmulpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 89, 46], OperandSize::Dword)
}

fn vmulpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 649646619, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 217, 89, 52, 149, 27, 210, 184, 38], OperandSize::Dword)
}

fn vmulpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 237, 190, 89, 234], OperandSize::Qword)
}

fn vmulpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 864386950, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 204, 89, 140, 153, 134, 127, 133, 51], OperandSize::Qword)
}

fn vmulpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1740553684, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 157, 210, 89, 4, 69, 212, 189, 190, 103], OperandSize::Qword)
}

