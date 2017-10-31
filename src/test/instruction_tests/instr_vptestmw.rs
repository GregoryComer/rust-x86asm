use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 9, 38, 238], OperandSize::Dword)
}

#[test]
fn vptestmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1078075760, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 11, 38, 60, 197, 112, 33, 66, 64], OperandSize::Dword)
}

#[test]
fn vptestmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 3, 38, 217], OperandSize::Qword)
}

#[test]
fn vptestmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1227742088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 149, 6, 38, 172, 81, 136, 219, 45, 73], OperandSize::Qword)
}

#[test]
fn vptestmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 45, 38, 244], OperandSize::Dword)
}

#[test]
fn vptestmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 637731423, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 41, 38, 140, 94, 95, 2, 3, 38], OperandSize::Dword)
}

#[test]
fn vptestmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 237, 44, 38, 251], OperandSize::Qword)
}

#[test]
fn vptestmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 36, 38, 15], OperandSize::Qword)
}

#[test]
fn vptestmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 75, 38, 225], OperandSize::Dword)
}

#[test]
fn vptestmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 73, 38, 60, 122], OperandSize::Dword)
}

#[test]
fn vptestmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 181, 67, 38, 247], OperandSize::Qword)
}

#[test]
fn vptestmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2028368700, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 189, 74, 38, 164, 241, 60, 115, 230, 120], OperandSize::Qword)
}

