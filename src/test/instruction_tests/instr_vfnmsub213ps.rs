use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 174, 210], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 174, 44, 186], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 174, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 174, 60, 208], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 174, 219], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 174, 40], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 174, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 1128276095, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 174, 177, 127, 32, 64, 67], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 174, 200], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 513445955, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 174, 172, 219, 67, 144, 154, 30], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 2007196670, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 174, 172, 251, 254, 99, 163, 119], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 45, 131, 174, 243], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 275216867, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 61, 137, 174, 12, 189, 227, 121, 103, 16], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1721088123, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 151, 174, 44, 205, 123, 184, 149, 102], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 174, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 174, 38], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1503892488, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 190, 174, 12, 205, 8, 148, 163, 89], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 165, 174, 254], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 174, 17], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RSI, 628964403, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 178, 174, 150, 51, 60, 125, 37], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 187, 174, 226], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 174, 22], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 220, 174, 52, 247], OperandSize::Dword)
}

#[test]
fn vfnmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 13, 190, 174, 251], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 2065616052, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 174, 188, 73, 180, 204, 30, 123], OperandSize::Qword)
}

#[test]
fn vfnmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 35219509, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 213, 174, 140, 113, 53, 104, 25, 2], OperandSize::Qword)
}

