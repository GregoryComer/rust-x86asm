use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 127, 219], OperandSize::Dword)
}

#[test]
fn vpermt2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1923650449, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 127, 188, 143, 145, 147, 168, 114], OperandSize::Dword)
}

#[test]
fn vpermt2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1161493734, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 127, 156, 80, 230, 252, 58, 69], OperandSize::Dword)
}

#[test]
fn vpermt2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 157, 143, 127, 239], OperandSize::Qword)
}

#[test]
fn vpermt2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 625127998, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 133, 127, 4, 213, 62, 178, 66, 37], OperandSize::Qword)
}

#[test]
fn vpermt2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 157, 159, 127, 34], OperandSize::Qword)
}

#[test]
fn vpermt2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 127, 212], OperandSize::Dword)
}

#[test]
fn vpermt2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 170, 127, 6], OperandSize::Dword)
}

#[test]
fn vpermt2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 189, 127, 35], OperandSize::Dword)
}

#[test]
fn vpermt2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 157, 171, 127, 236], OperandSize::Qword)
}

#[test]
fn vpermt2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1706456463, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 181, 163, 127, 188, 151, 143, 117, 182, 101], OperandSize::Qword)
}

#[test]
fn vpermt2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 173, 181, 127, 39], OperandSize::Qword)
}

#[test]
fn vpermt2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 127, 254], OperandSize::Dword)
}

#[test]
fn vpermt2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 127, 55], OperandSize::Dword)
}

#[test]
fn vpermt2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 1487310625, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 127, 158, 33, 143, 166, 88], OperandSize::Dword)
}

#[test]
fn vpermt2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 237, 196, 127, 200], OperandSize::Qword)
}

#[test]
fn vpermt2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 157, 202, 127, 60, 146], OperandSize::Qword)
}

#[test]
fn vpermt2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 189, 221, 127, 52, 78], OperandSize::Qword)
}

