use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 11, 38, 234], OperandSize::Dword)
}

fn vptestmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 279500418, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 9, 38, 60, 197, 130, 214, 168, 16], OperandSize::Dword)
}

fn vptestmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 5, 11, 38, 218], OperandSize::Qword)
}

fn vptestmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1858375950, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 13, 38, 44, 125, 14, 145, 196, 110], OperandSize::Qword)
}

fn vptestmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 41, 38, 246], OperandSize::Dword)
}

fn vptestmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 158486718, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 42, 38, 148, 177, 190, 80, 114, 9], OperandSize::Dword)
}

fn vptestmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 37, 35, 38, 232], OperandSize::Qword)
}

fn vptestmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1912766911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 38, 38, 148, 179, 191, 129, 2, 114], OperandSize::Qword)
}

fn vptestmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 75, 38, 244], OperandSize::Dword)
}

fn vptestmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 74, 38, 25], OperandSize::Dword)
}

fn vptestmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 45, 79, 38, 235], OperandSize::Qword)
}

fn vptestmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1738874002, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 71, 38, 156, 119, 146, 28, 165, 103], OperandSize::Qword)
}

