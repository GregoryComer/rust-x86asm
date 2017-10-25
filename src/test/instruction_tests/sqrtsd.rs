use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 202], OperandSize::Dword)
}

fn sqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 1448890313, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 155, 201, 79, 92, 86], OperandSize::Dword)
}

fn sqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 241], OperandSize::Qword)
}

fn sqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 28, 215], OperandSize::Qword)
}

