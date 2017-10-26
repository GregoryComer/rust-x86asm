use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 127, 227], OperandSize::Dword)
}

#[test]
fn vpermt2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 474948569, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 127, 52, 181, 217, 35, 79, 28], OperandSize::Dword)
}

#[test]
fn vpermt2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1694752106, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 159, 127, 172, 193, 106, 221, 3, 101], OperandSize::Dword)
}

#[test]
fn vpermt2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 85, 132, 127, 203], OperandSize::Qword)
}

#[test]
fn vpermt2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 77, 130, 127, 62], OperandSize::Qword)
}

#[test]
fn vpermt2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RBX, 1130441165, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 151, 127, 147, 205, 41, 97, 67], OperandSize::Qword)
}

#[test]
fn vpermt2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 127, 219], OperandSize::Dword)
}

#[test]
fn vpermt2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 2027906247, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 127, 169, 199, 100, 223, 120], OperandSize::Dword)
}

#[test]
fn vpermt2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 189, 127, 12, 155], OperandSize::Dword)
}

#[test]
fn vpermt2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 109, 165, 127, 198], OperandSize::Qword)
}

#[test]
fn vpermt2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 998278869, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 5, 172, 127, 180, 214, 213, 134, 128, 59], OperandSize::Qword)
}

#[test]
fn vpermt2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1646785517, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 177, 127, 148, 64, 237, 243, 39, 98], OperandSize::Qword)
}

#[test]
fn vpermt2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 127, 245], OperandSize::Dword)
}

#[test]
fn vpermt2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1834111740, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 201, 127, 128, 252, 82, 82, 109], OperandSize::Dword)
}

#[test]
fn vpermt2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 1578381793, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 220, 127, 144, 225, 49, 20, 94], OperandSize::Dword)
}

#[test]
fn vpermt2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 13, 197, 127, 249], OperandSize::Qword)
}

#[test]
fn vpermt2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 93, 203, 127, 44, 246], OperandSize::Qword)
}

#[test]
fn vpermt2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 85, 213, 127, 36, 154], OperandSize::Qword)
}

