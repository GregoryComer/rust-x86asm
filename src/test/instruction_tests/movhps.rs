use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 32424642, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 147, 194, 194, 238, 1], OperandSize::Dword)
}

fn movhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1050193998, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 12, 93, 78, 176, 152, 62], OperandSize::Qword)
}

fn movhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 20, 249], OperandSize::Dword)
}

fn movhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectDisplaced(RSI, 1872549594, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 174, 218, 214, 156, 111], OperandSize::Qword)
}

