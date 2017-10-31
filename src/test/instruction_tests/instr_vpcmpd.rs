use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 10, 31, 241, 92], OperandSize::Dword)
}

#[test]
fn vpcmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1672119407, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 9, 31, 148, 126, 111, 132, 170, 99, 49], OperandSize::Dword)
}

#[test]
fn vpcmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 25, 31, 54, 27], OperandSize::Dword)
}

#[test]
fn vpcmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 29, 7, 31, 224, 6], OperandSize::Qword)
}

#[test]
fn vpcmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 34679219, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 12, 31, 148, 127, 179, 41, 17, 2, 14], OperandSize::Qword)
}

#[test]
fn vpcmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 41715837, Some(OperandSize::Dword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 28, 31, 52, 69, 125, 136, 124, 2, 22], OperandSize::Qword)
}

#[test]
fn vpcmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 43, 31, 220, 85], OperandSize::Dword)
}

#[test]
fn vpcmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1865588760, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 42, 31, 145, 24, 160, 50, 111, 9], OperandSize::Dword)
}

#[test]
fn vpcmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 206485512, Some(OperandSize::Dword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 61, 31, 12, 157, 8, 184, 78, 12, 86], OperandSize::Dword)
}

#[test]
fn vpcmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 39, 31, 238, 102], OperandSize::Qword)
}

#[test]
fn vpcmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 13, 38, 31, 55, 122], OperandSize::Qword)
}

#[test]
fn vpcmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 53, 49, 31, 8, 81], OperandSize::Qword)
}

#[test]
fn vpcmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 75, 31, 251, 12], OperandSize::Dword)
}

#[test]
fn vpcmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 524486333, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 76, 31, 156, 146, 189, 6, 67, 31, 11], OperandSize::Dword)
}

#[test]
fn vpcmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 93, 31, 20, 127, 117], OperandSize::Dword)
}

#[test]
fn vpcmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 53, 65, 31, 247, 54], OperandSize::Qword)
}

#[test]
fn vpcmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1106731316, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 70, 31, 188, 207, 52, 97, 247, 65, 47], OperandSize::Qword)
}

#[test]
fn vpcmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1935269664, Some(OperandSize::Dword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 84, 31, 140, 66, 32, 223, 89, 115, 45], OperandSize::Qword)
}

