use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 127, 249], OperandSize::Dword)
}

#[test]
fn vpermt2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1629639680, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 127, 172, 137, 0, 84, 34, 97], OperandSize::Dword)
}

#[test]
fn vpermt2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 967957308, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 127, 44, 205, 60, 219, 177, 57], OperandSize::Dword)
}

#[test]
fn vpermt2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 21, 129, 127, 202], OperandSize::Qword)
}

#[test]
fn vpermt2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RCX, 344642830, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 45, 139, 127, 153, 14, 213, 138, 20], OperandSize::Qword)
}

#[test]
fn vpermt2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 784282348, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 158, 127, 4, 117, 236, 50, 191, 46], OperandSize::Qword)
}

#[test]
fn vpermt2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 127, 235], OperandSize::Dword)
}

#[test]
fn vpermt2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 55635549, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 127, 166, 93, 238, 80, 3], OperandSize::Dword)
}

#[test]
fn vpermt2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 185, 127, 55], OperandSize::Dword)
}

#[test]
fn vpermt2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 69, 161, 127, 197], OperandSize::Qword)
}

#[test]
fn vpermt2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1704205663, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 117, 172, 127, 159, 95, 29, 148, 101], OperandSize::Qword)
}

#[test]
fn vpermt2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 93, 183, 127, 59], OperandSize::Qword)
}

#[test]
fn vpermt2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 127, 198], OperandSize::Dword)
}

#[test]
fn vpermt2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 869009861, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 127, 180, 198, 197, 9, 204, 51], OperandSize::Dword)
}

#[test]
fn vpermt2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1924636239, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 127, 137, 79, 158, 183, 114], OperandSize::Dword)
}

#[test]
fn vpermt2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 109, 198, 127, 247], OperandSize::Qword)
}

#[test]
fn vpermt2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 61, 195, 127, 12, 72], OperandSize::Qword)
}

#[test]
fn vpermt2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 808943880, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 61, 217, 127, 172, 210, 8, 129, 55, 48], OperandSize::Qword)
}

