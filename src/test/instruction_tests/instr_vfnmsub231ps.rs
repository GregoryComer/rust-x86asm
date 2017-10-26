use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 190, 192], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 190, 48], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 246], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 578481950, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 172, 79, 30, 239, 122, 34], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 190, 193], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1904546319, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 190, 172, 67, 15, 18, 133, 113], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 190, 214], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 190, 11], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 190, 214], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 190, 60, 87], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1713486347, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 156, 190, 44, 93, 11, 186, 33, 102], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 85, 138, 190, 216], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 139, 190, 20, 240], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1185431989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 29, 156, 190, 188, 121, 181, 65, 168, 70], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 190, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1316839598, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 190, 172, 217, 174, 96, 125, 78], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1966690637, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 190, 28, 189, 77, 81, 57, 117], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 29, 169, 190, 243], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1856733448, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 174, 190, 60, 221, 8, 129, 171, 110], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 894989367, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 29, 187, 190, 164, 126, 55, 116, 88, 53], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 252, 190, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2068006605, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 190, 140, 131, 205, 70, 67, 123], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 524920732, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 190, 44, 117, 156, 167, 73, 31], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 37, 243, 190, 211], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 925447848, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 21, 202, 190, 156, 203, 168, 54, 41, 55], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 453962481, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 37, 221, 190, 156, 185, 241, 234, 14, 27], OperandSize::Qword)
}

