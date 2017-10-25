use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 1143404505, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 140, 139, 217, 247, 38, 68], OperandSize::Dword)
}

fn vpmaskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 140, 23], OperandSize::Qword)
}

fn vpmaskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1185590779, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 140, 161, 251, 173, 170, 70], OperandSize::Dword)
}

fn vpmaskmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 140, 28, 218], OperandSize::Qword)
}

fn vpmaskmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1596637756, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 142, 188, 182, 60, 194, 42, 95], OperandSize::Dword)
}

fn vpmaskmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(RSI, Two, 925685556, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 142, 44, 117, 52, 215, 44, 55], OperandSize::Qword)
}

fn vpmaskmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 7900800, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 142, 44, 205, 128, 142, 120, 0], OperandSize::Dword)
}

fn vpmaskmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1106591054, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 142, 4, 245, 78, 61, 245, 65], OperandSize::Qword)
}

