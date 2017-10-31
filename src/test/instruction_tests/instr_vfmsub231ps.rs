use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 186, 205], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 163666460, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 186, 36, 69, 28, 90, 193, 9], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 186, 254], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDI, 164123637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 186, 159, 245, 83, 200, 9], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 186, 222], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 650639570, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 186, 164, 202, 210, 248, 199, 38], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 186, 193], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 1960284912, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 186, 183, 240, 146, 215, 116], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 186, 203], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 186, 20, 86], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 1206314271, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 154, 186, 163, 31, 229, 230, 71], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 69, 134, 186, 212], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2054881369, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 29, 129, 186, 4, 85, 89, 0, 123, 122], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 13, 156, 186, 28, 74], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 186, 227], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 137232408, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 186, 140, 159, 24, 0, 46, 8], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 188, 186, 52, 223], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 69, 165, 186, 210], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RDX, 2071028614, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 29, 170, 186, 162, 134, 99, 113, 123], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 611688072, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 185, 186, 140, 134, 136, 158, 117, 36], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 186, 186, 246], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 2034982401, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 186, 143, 1, 94, 75, 121], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 218, 186, 12, 65], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 13, 145, 186, 210], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RBX, 1717986780, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 29, 196, 186, 155, 220, 101, 102, 102], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 220, 186, 54], OperandSize::Qword)
}

