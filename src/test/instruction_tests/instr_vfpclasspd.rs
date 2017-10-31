use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 15, 102, 242, 89], OperandSize::Dword)
}

#[test]
fn vfpclasspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 11, 102, 28, 152, 51], OperandSize::Dword)
}

#[test]
fn vfpclasspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 30, 102, 60, 89, 10], OperandSize::Dword)
}

#[test]
fn vfpclasspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 102, 222, 14], OperandSize::Qword)
}

#[test]
fn vfpclasspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 15, 102, 60, 192, 127], OperandSize::Qword)
}

#[test]
fn vfpclasspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 25, 102, 12, 95, 9], OperandSize::Qword)
}

#[test]
fn vfpclasspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 47, 102, 240, 91], OperandSize::Dword)
}

#[test]
fn vfpclasspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 102, 9, 95], OperandSize::Dword)
}

#[test]
fn vfpclasspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 58, 102, 44, 249, 85], OperandSize::Dword)
}

#[test]
fn vfpclasspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 253, 46, 102, 201, 28], OperandSize::Qword)
}

#[test]
fn vfpclasspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 244689025, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 47, 102, 28, 69, 129, 168, 149, 14, 61], OperandSize::Qword)
}

#[test]
fn vfpclasspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 63, 102, 62, 71], OperandSize::Qword)
}

#[test]
fn vfpclasspd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 75, 102, 250, 28], OperandSize::Dword)
}

#[test]
fn vfpclasspd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 102, 59, 101], OperandSize::Dword)
}

#[test]
fn vfpclasspd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 89, 102, 44, 86, 91], OperandSize::Dword)
}

#[test]
fn vfpclasspd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 253, 74, 102, 225, 72], OperandSize::Qword)
}

#[test]
fn vfpclasspd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 595628868, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 73, 102, 28, 181, 68, 147, 128, 35, 21], OperandSize::Qword)
}

#[test]
fn vfpclasspd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectDisplaced(RDX, 412665818, Some(OperandSize::Qword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 93, 102, 186, 218, 199, 152, 24, 53], OperandSize::Qword)
}

