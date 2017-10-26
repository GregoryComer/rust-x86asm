use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 186, 216], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 186, 3], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 186, 245], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 870249339, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 186, 12, 253, 123, 243, 222, 51], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 186, 233], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 719729899, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 186, 178, 235, 52, 230, 42], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 186, 249], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 307430732, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 186, 12, 69, 76, 5, 83, 18], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 186, 254], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1534168696, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 186, 140, 122, 120, 142, 113, 91], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1504338176, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 186, 172, 208, 0, 97, 170, 89], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 85, 141, 186, 213], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 645075219, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 101, 133, 186, 36, 93, 19, 17, 115, 38], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RDX, 1730860508, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 13, 155, 186, 170, 220, 213, 42, 103], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 186, 201], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 186, 12, 75], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 2106589399, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 188, 186, 164, 139, 215, 0, 144, 125], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 13, 163, 186, 250], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 5, 167, 186, 56], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 37, 191, 186, 44, 190], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 190, 186, 239], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 148687227, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 186, 161, 123, 201, 220, 8], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 217, 186, 39], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 29, 221, 186, 229], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1996260051, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 77, 194, 186, 188, 144, 211, 130, 252, 118], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 2005152159, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 93, 211, 186, 156, 192, 159, 49, 132, 119], OperandSize::Qword)
}

