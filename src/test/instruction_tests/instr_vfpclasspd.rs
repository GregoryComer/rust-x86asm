use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 10, 102, 225, 99], OperandSize::Dword)
}

#[test]
fn vfpclasspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1581897083, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 102, 20, 125, 123, 213, 73, 94, 108], OperandSize::Dword)
}

#[test]
fn vfpclasspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 31, 102, 12, 190, 80], OperandSize::Dword)
}

#[test]
fn vfpclasspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 253, 14, 102, 227, 15], OperandSize::Qword)
}

#[test]
fn vfpclasspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 697589625, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 15, 102, 60, 221, 121, 95, 148, 41, 56], OperandSize::Qword)
}

#[test]
fn vfpclasspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(RAX, 1549910294, Some(OperandSize::Qword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 25, 102, 152, 22, 193, 97, 92, 89], OperandSize::Qword)
}

#[test]
fn vfpclasspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 47, 102, 244, 15], OperandSize::Dword)
}

#[test]
fn vfpclasspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 42, 102, 14, 119], OperandSize::Dword)
}

#[test]
fn vfpclasspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 968040249, Some(OperandSize::Qword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 57, 102, 28, 197, 57, 31, 179, 57, 72], OperandSize::Dword)
}

#[test]
fn vfpclasspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM11)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 253, 43, 102, 251, 75], OperandSize::Qword)
}

#[test]
fn vfpclasspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 2113183070, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 42, 102, 180, 222, 94, 157, 244, 125, 84], OperandSize::Qword)
}

#[test]
fn vfpclasspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 301149253, Some(OperandSize::Qword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 59, 102, 188, 65, 69, 44, 243, 17, 80], OperandSize::Qword)
}

#[test]
fn vfpclasspd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 76, 102, 252, 113], OperandSize::Dword)
}

#[test]
fn vfpclasspd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 407645767, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 77, 102, 180, 82, 71, 46, 76, 24, 76], OperandSize::Dword)
}

#[test]
fn vfpclasspd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 95, 102, 20, 191, 44], OperandSize::Dword)
}

#[test]
fn vfpclasspd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 253, 78, 102, 237, 19], OperandSize::Qword)
}

#[test]
fn vfpclasspd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1764421957, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 76, 102, 156, 145, 69, 241, 42, 105, 6], OperandSize::Qword)
}

#[test]
fn vfpclasspd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1956835058, Some(OperandSize::Qword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 93, 102, 52, 141, 242, 238, 162, 116, 45], OperandSize::Qword)
}

