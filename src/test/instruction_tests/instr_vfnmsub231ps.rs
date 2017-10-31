use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1279959661, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 190, 4, 245, 109, 162, 74, 76], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 190, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1687677034, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 12, 213, 106, 232, 151, 100], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 190, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 190, 28, 155], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 190, 222], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 190, 12, 123], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 190, 238], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 701289787, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 190, 44, 221, 59, 213, 204, 41], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 155, 190, 62], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 85, 134, 190, 236], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RAX, 1356921697, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 45, 131, 190, 168, 97, 251, 224, 80], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 117, 149, 190, 2], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 190, 241], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1277331518, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 190, 188, 192, 62, 136, 34, 76], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 185, 190, 44, 247], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 13, 175, 190, 197], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 166, 190, 60, 136], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM13)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 185, 190, 19], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 187, 190, 229], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2094949119, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 205, 190, 44, 245, 255, 98, 222, 124], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 220, 190, 4, 112], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 5, 145, 190, 215], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 85, 196, 190, 20, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 421850057, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 109, 215, 190, 20, 141, 201, 235, 36, 25], OperandSize::Qword)
}

