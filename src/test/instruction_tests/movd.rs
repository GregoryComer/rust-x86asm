use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM1)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 203], OperandSize::Dword)
}

fn movd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1913969562, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 188, 203, 154, 219, 20, 114], OperandSize::Dword)
}

fn movd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM0)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 199], OperandSize::Qword)
}

fn movd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 50], OperandSize::Qword)
}

fn movd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 203], OperandSize::Dword)
}

fn movd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 36, 223], OperandSize::Dword)
}

fn movd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 218], OperandSize::Qword)
}

fn movd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 544698206, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 140, 154, 94, 111, 119, 32], OperandSize::Qword)
}

fn movd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 234], OperandSize::Dword)
}

fn movd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EBX, 648062226, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 187, 18, 165, 160, 38], OperandSize::Dword)
}

fn movd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 247], OperandSize::Qword)
}

fn movd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(RSI, 1820181306, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 174, 58, 195, 125, 108], OperandSize::Qword)
}

fn movd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 237], OperandSize::Dword)
}

fn movd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 22], OperandSize::Dword)
}

fn movd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 253], OperandSize::Qword)
}

fn movd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1191862946, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 12, 221, 162, 98, 10, 71], OperandSize::Qword)
}

