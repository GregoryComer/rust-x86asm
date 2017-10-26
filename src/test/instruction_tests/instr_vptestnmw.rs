use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 206, 14, 38, 244], OperandSize::Dword)
}

#[test]
fn vptestnmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 987299373, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 254, 14, 38, 12, 213, 45, 254, 216, 58], OperandSize::Dword)
}

#[test]
fn vptestnmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 254, 13, 38, 203], OperandSize::Qword)
}

#[test]
fn vptestnmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1914904118, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 238, 12, 38, 28, 85, 54, 30, 35, 114], OperandSize::Qword)
}

#[test]
fn vptestnmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 254, 45, 38, 209], OperandSize::Dword)
}

#[test]
fn vptestnmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1559503848, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 222, 46, 38, 148, 73, 232, 35, 244, 92], OperandSize::Dword)
}

#[test]
fn vptestnmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 174, 45, 38, 206], OperandSize::Qword)
}

#[test]
fn vptestnmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RDI, 815661377, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 134, 41, 38, 183, 65, 1, 158, 48], OperandSize::Qword)
}

#[test]
fn vptestnmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 246, 74, 38, 221], OperandSize::Dword)
}

#[test]
fn vptestnmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 238, 75, 38, 51], OperandSize::Dword)
}

#[test]
fn vptestnmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 182, 73, 38, 219], OperandSize::Qword)
}

#[test]
fn vptestnmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 2000552758, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 190, 70, 38, 172, 66, 54, 3, 62, 119], OperandSize::Qword)
}

