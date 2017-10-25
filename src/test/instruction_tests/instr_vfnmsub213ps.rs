use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 213], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 159368046, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 174, 180, 66, 110, 195, 127, 9], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 244], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 58], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 174, 206], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1825963630, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 174, 172, 151, 110, 254, 213, 108], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 174, 207], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 808936349, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 174, 36, 117, 157, 99, 55, 48], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 174, 227], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 174, 0], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 344991504, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 153, 174, 177, 16, 39, 144, 20], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 5, 143, 174, 213], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 83913299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 109, 138, 174, 146, 83, 106, 0, 5], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDI, 703107472, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 148, 174, 191, 144, 145, 232, 41], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 174, 249], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 174, 44, 182], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 186, 174, 28, 78], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 117, 169, 174, 246], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 21, 171, 174, 20, 87], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 5, 177, 174, 14], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 185, 174, 201], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1593053772, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 174, 20, 205, 76, 18, 244, 94], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 221, 174, 55], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 109, 221, 174, 195], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 45, 197, 174, 36, 214], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 21, 214, 174, 35], OperandSize::Qword)
}

