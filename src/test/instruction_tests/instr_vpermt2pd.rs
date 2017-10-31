use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 127, 232], OperandSize::Dword)
}

#[test]
fn vpermt2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 406598599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 127, 166, 199, 51, 60, 24], OperandSize::Dword)
}

#[test]
fn vpermt2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1862672604, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 154, 127, 188, 118, 220, 32, 6, 111], OperandSize::Dword)
}

#[test]
fn vpermt2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 134, 127, 195], OperandSize::Qword)
}

#[test]
fn vpermt2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 627545578, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 141, 131, 127, 172, 154, 234, 149, 103, 37], OperandSize::Qword)
}

#[test]
fn vpermt2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 197, 155, 127, 11], OperandSize::Qword)
}

#[test]
fn vpermt2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 127, 204], OperandSize::Dword)
}

#[test]
fn vpermt2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 798338203, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 127, 44, 221, 155, 172, 149, 47], OperandSize::Dword)
}

#[test]
fn vpermt2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 191, 127, 58], OperandSize::Dword)
}

#[test]
fn vpermt2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 127, 239], OperandSize::Qword)
}

#[test]
fn vpermt2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 271691291, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 149, 167, 127, 12, 205, 27, 174, 49, 16], OperandSize::Qword)
}

#[test]
fn vpermt2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RDI, 376610567, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 188, 127, 151, 7, 159, 114, 22], OperandSize::Qword)
}

#[test]
fn vpermt2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 127, 210], OperandSize::Dword)
}

#[test]
fn vpermt2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 127, 20, 217], OperandSize::Dword)
}

#[test]
fn vpermt2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 127, 20, 192], OperandSize::Dword)
}

#[test]
fn vpermt2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 205, 206, 127, 243], OperandSize::Qword)
}

#[test]
fn vpermt2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RCX, 1982036281, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 157, 197, 127, 145, 57, 121, 35, 118], OperandSize::Qword)
}

#[test]
fn vpermt2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RBX, 649210132, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 221, 215, 127, 163, 20, 41, 178, 38], OperandSize::Qword)
}

