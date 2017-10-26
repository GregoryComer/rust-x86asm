use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 190, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 190, 4, 193], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 190, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDI, 1085161198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 190, 135, 238, 62, 174, 64], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 246], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 79559637, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 28, 69, 213, 251, 189, 4], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 190, 215], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 190, 36, 115], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 190, 235], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 190, 42], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 156, 190, 57], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 213, 139, 190, 246], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 189, 142, 190, 12, 134], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 981819345, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 245, 158, 190, 183, 209, 95, 133, 58], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 190, 246], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 190, 60, 143], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 189, 190, 48], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 173, 167, 190, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 149, 161, 190, 12, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 30405045, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 237, 185, 190, 60, 117, 181, 241, 207, 1], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 156, 190, 222], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 2144283344, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 190, 187, 208, 42, 207, 127], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 190, 28, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 197, 220, 190, 228], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 245, 203, 190, 0], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 887306267, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 214, 190, 188, 139, 27, 56, 227, 52], OperandSize::Qword)
}

