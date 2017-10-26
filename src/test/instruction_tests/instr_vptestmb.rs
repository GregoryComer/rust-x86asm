use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 15, 38, 237], OperandSize::Dword)
}

#[test]
fn vptestmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 323687914, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 10, 38, 28, 93, 234, 21, 75, 19], OperandSize::Dword)
}

#[test]
fn vptestmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 13, 9, 38, 241], OperandSize::Qword)
}

#[test]
fn vptestmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 478012856, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 45, 15, 38, 148, 247, 184, 229, 125, 28], OperandSize::Qword)
}

#[test]
fn vptestmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 45, 38, 203], OperandSize::Dword)
}

#[test]
fn vptestmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 45, 38, 20, 78], OperandSize::Dword)
}

#[test]
fn vptestmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 37, 34, 38, 229], OperandSize::Qword)
}

#[test]
fn vptestmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 35, 38, 59], OperandSize::Qword)
}

#[test]
fn vptestmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 73, 38, 249], OperandSize::Dword)
}

#[test]
fn vptestmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 975023732, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 74, 38, 44, 133, 116, 174, 29, 58], OperandSize::Dword)
}

#[test]
fn vptestmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 29, 70, 38, 227], OperandSize::Qword)
}

#[test]
fn vptestmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1343627886, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 45, 71, 38, 180, 72, 110, 34, 22, 80], OperandSize::Qword)
}

