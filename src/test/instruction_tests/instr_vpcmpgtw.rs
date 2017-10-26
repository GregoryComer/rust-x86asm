use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 101, 205], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 44, 127], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 101, 196], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 1878441379, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 101, 160, 163, 189, 246, 111], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 101, 244], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 354607673, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 101, 156, 113, 57, 226, 34, 21], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 101, 196], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1721260942, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 101, 44, 141, 142, 91, 152, 102], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 14, 101, 226], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 11, 101, 44, 190], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 13, 12, 101, 225], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 10, 101, 20, 183], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 44, 101, 219], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 1272115087, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 41, 101, 190, 143, 239, 210, 75], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 29, 46, 101, 200], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RAX, 1425014034, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 35, 101, 160, 18, 253, 239, 84], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 75, 101, 252], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 74, 101, 26], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 85, 66, 101, 208], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 77, 101, 19], OperandSize::Qword)
}

