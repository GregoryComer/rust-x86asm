use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 202], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 2108618563, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 101, 180, 119, 67, 247, 174, 125], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 242], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 101, 55], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 101, 210], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 101, 46], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 101, 194], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 101, 60, 128], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 13, 101, 208], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 14, 101, 44, 222], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 85, 12, 101, 236], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 6, 101, 60, 151], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 45, 101, 226], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 512220104, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 44, 101, 60, 245, 200, 219, 135, 30], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 125, 33, 101, 212], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 2054908220, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 45, 37, 101, 148, 199, 60, 105, 123, 122], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 79, 101, 235], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 2111506637, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 74, 101, 164, 201, 205, 8, 219, 125], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 68, 101, 230], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 173503162, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 71, 101, 172, 190, 186, 114, 87, 10], OperandSize::Qword)
}

