use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 174, 221], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 174, 27], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 174, 224], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 4, 142], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 174, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 174, 11], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 174, 251], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 174, 60, 142], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 174, 199], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 1125483759, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 174, 154, 239, 132, 21, 67], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1420096460, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 158, 174, 140, 75, 204, 243, 164, 84], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 69, 135, 174, 212], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 634846230, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 174, 164, 135, 22, 252, 214, 37], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 85, 155, 174, 50], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 174, 194], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 290014619, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 174, 188, 222, 155, 69, 73, 17], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 174, 20, 130], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 101, 164, 174, 243], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 173, 174, 49], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 45, 185, 174, 51], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 188, 174, 216], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1619295215, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 174, 188, 72, 239, 123, 132, 96], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 174, 44, 131], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 109, 210, 174, 192], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2097389249, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 199, 174, 12, 141, 193, 158, 3, 125], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 21, 214, 174, 49], OperandSize::Qword)
}

