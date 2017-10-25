use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 207], OperandSize::Dword)
}

fn pmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 25], OperandSize::Dword)
}

fn pmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 226], OperandSize::Qword)
}

fn pmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 44, 86], OperandSize::Qword)
}

fn pmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 206], OperandSize::Dword)
}

fn pmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 166774652, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 52, 117, 124, 199, 240, 9], OperandSize::Dword)
}

fn pmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 241], OperandSize::Qword)
}

fn pmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1092148944, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 20, 93, 208, 222, 24, 65], OperandSize::Qword)
}

