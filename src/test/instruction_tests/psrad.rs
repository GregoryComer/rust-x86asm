use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 226, 68], OperandSize::Dword)
}

fn psrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 226, 78], OperandSize::Qword)
}

fn psrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 231, 80], OperandSize::Dword)
}

fn psrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 225, 54], OperandSize::Qword)
}

fn psrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 193], OperandSize::Dword)
}

fn psrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1069710431, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 20, 77, 95, 124, 194, 63], OperandSize::Dword)
}

fn psrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 232], OperandSize::Qword)
}

fn psrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RSI, 571842352, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 134, 48, 159, 21, 34], OperandSize::Qword)
}

fn psrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 218], OperandSize::Dword)
}

fn psrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 1], OperandSize::Dword)
}

fn psrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 235], OperandSize::Qword)
}

fn psrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 28, 122], OperandSize::Qword)
}

