use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 194], OperandSize::Dword)
}

fn vpabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 489165264, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 128, 208, 17, 40, 29], OperandSize::Dword)
}

fn vpabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 227], OperandSize::Qword)
}

fn vpabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 47], OperandSize::Qword)
}

fn vpabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 199], OperandSize::Dword)
}

fn vpabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 1087832104, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 175, 40, 0, 215, 64], OperandSize::Dword)
}

fn vpabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 226], OperandSize::Qword)
}

fn vpabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1160095879, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 4, 253, 135, 168, 37, 69], OperandSize::Qword)
}

