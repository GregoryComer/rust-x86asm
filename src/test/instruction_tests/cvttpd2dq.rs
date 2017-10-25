use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 222], OperandSize::Dword)
}

fn cvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 430269718, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 172, 78, 22, 101, 165, 25], OperandSize::Dword)
}

fn cvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 244], OperandSize::Qword)
}

fn cvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1961118463, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 20, 253, 255, 74, 228, 116], OperandSize::Qword)
}

