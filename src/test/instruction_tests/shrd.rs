use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 219, 66], OperandSize::Word)
}

fn shrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 20, 112], OperandSize::Word)
}

fn shrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 253, 54], OperandSize::Dword)
}

fn shrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 2015883486, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 44, 77, 222, 240, 39, 120, 122], OperandSize::Dword)
}

fn shrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 217, 96], OperandSize::Qword)
}

fn shrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 44, 66, 81], OperandSize::Qword)
}

fn shrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 254, 43], OperandSize::Word)
}

fn shrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 59, 118], OperandSize::Word)
}

fn shrd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 250, 49], OperandSize::Dword)
}

fn shrd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 39, 113], OperandSize::Dword)
}

fn shrd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 206, 67], OperandSize::Qword)
}

fn shrd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1208093736, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 156, 81, 40, 12, 2, 72, 62], OperandSize::Qword)
}

fn shrd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 239, 122], OperandSize::Qword)
}

fn shrd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 20, 193, 95], OperandSize::Qword)
}

fn shrd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 225], OperandSize::Word)
}

fn shrd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(SI, 31001, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 172, 25, 121], OperandSize::Word)
}

fn shrd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 239], OperandSize::Dword)
}

fn shrd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 695833679, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 140, 134, 79, 148, 121, 41], OperandSize::Dword)
}

fn shrd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 219], OperandSize::Qword)
}

fn shrd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 20, 209], OperandSize::Qword)
}

fn shrd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 217], OperandSize::Word)
}

fn shrd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(SI, 162, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 172, 162, 0], OperandSize::Word)
}

fn shrd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 222], OperandSize::Dword)
}

fn shrd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 58], OperandSize::Dword)
}

fn shrd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 206], OperandSize::Qword)
}

fn shrd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 18], OperandSize::Qword)
}

fn shrd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 230], OperandSize::Qword)
}

fn shrd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RCX, 325689840, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 161, 240, 161, 105, 19], OperandSize::Qword)
}

