use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vlddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 181397657, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 177, 153, 232, 207, 10], OperandSize::Dword)
}

fn vlddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 28, 242], OperandSize::Qword)
}

fn vlddqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1154582583, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 44, 141, 55, 136, 209, 68], OperandSize::Dword)
}

fn vlddqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RSI, 472428778, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 158, 234, 176, 40, 28], OperandSize::Qword)
}

