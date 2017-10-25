use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 222, 9, 38, 251], OperandSize::Dword)
}

#[test]
fn vptestnmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1523992767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 206, 14, 38, 148, 249, 191, 72, 214, 90], OperandSize::Dword)
}

#[test]
fn vptestnmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 166, 1, 38, 251], OperandSize::Qword)
}

#[test]
fn vptestnmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 158, 14, 38, 10], OperandSize::Qword)
}

#[test]
fn vptestnmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 214, 42, 38, 255], OperandSize::Dword)
}

#[test]
fn vptestnmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 254, 44, 38, 44, 144], OperandSize::Dword)
}

#[test]
fn vptestnmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 222, 43, 38, 239], OperandSize::Qword)
}

#[test]
fn vptestnmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RCX, 1773931510, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 238, 39, 38, 145, 246, 11, 188, 105], OperandSize::Qword)
}

#[test]
fn vptestnmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 206, 74, 38, 230], OperandSize::Dword)
}

#[test]
fn vptestnmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 246, 78, 38, 27], OperandSize::Dword)
}

#[test]
fn vptestnmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 198, 75, 38, 204], OperandSize::Qword)
}

#[test]
fn vptestnmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 692932053, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 230, 71, 38, 188, 190, 213, 77, 77, 41], OperandSize::Qword)
}

