use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 211, 19], OperandSize::Dword)
}

fn psrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM3)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 211, 96], OperandSize::Qword)
}

fn psrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 210, 59], OperandSize::Dword)
}

fn psrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 215, 51], OperandSize::Qword)
}

fn psrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 233], OperandSize::Dword)
}

fn psrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 310435165, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 172, 126, 93, 221, 128, 18], OperandSize::Dword)
}

fn psrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 201], OperandSize::Qword)
}

fn psrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 52, 187], OperandSize::Qword)
}

fn psrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 198], OperandSize::Dword)
}

fn psrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 960478537, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 142, 73, 189, 63, 57], OperandSize::Dword)
}

fn psrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 252], OperandSize::Qword)
}

fn psrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 60, 200], OperandSize::Qword)
}

