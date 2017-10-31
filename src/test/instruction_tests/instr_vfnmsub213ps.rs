use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 174, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1369996411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 174, 4, 245, 123, 124, 168, 81], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1362188592, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 174, 140, 187, 48, 89, 49, 81], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 174, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 174, 36, 143], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 174, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1791200211, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 174, 188, 90, 211, 139, 195, 106], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 174, 217], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 176023201, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 174, 156, 218, 161, 230, 125, 10], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 1880800384, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 174, 135, 128, 188, 26, 112], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 21, 129, 174, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 457098202, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 45, 142, 174, 132, 89, 218, 195, 62, 27], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RAX, 2018405103, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 147, 174, 128, 239, 106, 78, 120], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 174, 224], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 174, 60, 177], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 909463331, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 187, 174, 188, 209, 35, 79, 53, 54], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 13, 164, 174, 233], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1055868728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 29, 173, 174, 148, 203, 56, 71, 239, 62], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RAX, 344111304, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 61, 188, 174, 168, 200, 184, 130, 20], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 187, 174, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 174, 41], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 174, 4, 112], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 29, 178, 174, 235], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 69, 207, 174, 10], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 493710425, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 220, 174, 36, 157, 89, 108, 109, 29], OperandSize::Qword)
}

