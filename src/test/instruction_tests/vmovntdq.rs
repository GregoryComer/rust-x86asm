use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 31], OperandSize::Dword)
}

fn vmovntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1076627961, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 20, 133, 249, 9, 44, 64], OperandSize::Qword)
}

fn vmovntdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 33], OperandSize::Dword)
}

fn vmovntdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 12, 152], OperandSize::Qword)
}

fn vmovntdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EDI, 804369231, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 191, 79, 179, 241, 47], OperandSize::Dword)
}

fn vmovntdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 48], OperandSize::Qword)
}

fn vmovntdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 598275230, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 140, 250, 158, 244, 168, 35], OperandSize::Dword)
}

fn vmovntdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1800510375, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 125, 231, 148, 216, 167, 155, 81, 107], OperandSize::Qword)
}

fn vmovntdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1154905198, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 231, 20, 141, 110, 116, 214, 68], OperandSize::Dword)
}

fn vmovntdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(RBX, 2118916627, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 72, 231, 179, 19, 26, 76, 126], OperandSize::Qword)
}

