use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 117, 222], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 350377928, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 117, 132, 183, 200, 87, 226, 20], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 248], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 46], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 117, 228], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 1896767548, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 117, 135, 60, 96, 14, 113], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 117, 192], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 117, 10], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 10, 117, 251], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 11, 117, 52, 126], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 13, 117, 222], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1981774036, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 29, 9, 117, 140, 135, 212, 120, 31, 118], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 46, 117, 208], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1026783514, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 44, 117, 162, 26, 121, 51, 61], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 69, 46, 117, 235], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 21, 36, 117, 30], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 75, 117, 252], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 76, 117, 12, 215], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 117, 65, 117, 208], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RDI, 522304194, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 71, 117, 159, 194, 186, 33, 31], OperandSize::Qword)
}

