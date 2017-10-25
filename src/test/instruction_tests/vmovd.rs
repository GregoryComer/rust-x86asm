use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 196], OperandSize::Dword)
}

fn vmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 20, 246], OperandSize::Dword)
}

fn vmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 217], OperandSize::Qword)
}

fn vmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RSI, 1566955985, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 182, 209, 217, 101, 93], OperandSize::Qword)
}

fn vmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 199], OperandSize::Dword)
}

fn vmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 2108135989, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 12, 77, 53, 154, 167, 125], OperandSize::Dword)
}

fn vmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 110, 203], OperandSize::Qword)
}

fn vmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RAX, 1282294360, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 110, 152, 88, 66, 110, 76], OperandSize::Qword)
}

fn vmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 218], OperandSize::Dword)
}

fn vmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(EAX, 1465573498, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 136, 122, 224, 90, 87], OperandSize::Dword)
}

fn vmovd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 221], OperandSize::Qword)
}

fn vmovd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 14], OperandSize::Qword)
}

fn vmovd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 249], OperandSize::Dword)
}

fn vmovd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 36, 81], OperandSize::Dword)
}

fn vmovd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 233], OperandSize::Qword)
}

fn vmovd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(RSI, 231825170, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 166, 18, 95, 209, 13], OperandSize::Qword)
}

