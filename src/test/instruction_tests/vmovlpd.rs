use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 1444347584, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 18, 143, 192, 254, 22, 86], OperandSize::Dword)
}

fn vmovlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 18, 22], OperandSize::Qword)
}

fn vmovlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 18, 17], OperandSize::Dword)
}

fn vmovlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 189, 0, 18, 52, 94], OperandSize::Qword)
}

fn vmovlpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 221468868, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 20, 213, 196, 88, 51, 13], OperandSize::Dword)
}

fn vmovlpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1086435606, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 20, 77, 22, 177, 193, 64], OperandSize::Qword)
}

fn vmovlpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1594564892, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 4, 85, 28, 33, 11, 95], OperandSize::Dword)
}

fn vmovlpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 2046465010, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 19, 20, 141, 242, 147, 250, 121], OperandSize::Qword)
}

